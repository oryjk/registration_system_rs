use crate::application::user_service::UserService;
use crate::ports::user_repository::UserRepository;
use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::MultipartForm;
use actix_web::{web, Error, HttpResponse, Responder};
use base64::engine::general_purpose;
use base64::Engine;
use mime_guess::from_path;
use s3::creds::Credentials;
use s3::Bucket;
use std::fs;
use std::fs::File;

// 定义 S3 配置
#[derive(Debug, Clone)]
pub struct S3Config {
    pub region: String,
    pub bucket_name: String,
    pub access_key: String,
    pub secret_key: String,
}

#[derive(Debug, MultipartForm)]
pub struct UploadForm {
    #[multipart(limit = "100MB")]
    file: TempFile
}

async fn upload_to_s3(
    s3_config: &S3Config,
    file_path: &str,
    object_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let credentials = Credentials::new(
        Some(&s3_config.access_key),
        Some(&s3_config.secret_key),
        None,
        None,
        None,
    );

    let bucket = Bucket::new(&s3_config.bucket_name, s3_config.region.parse()?, credentials.unwrap()).unwrap();


    let file = fs::File::open(file_path)?;
    let metadata = file.metadata()?;
    let file_size = metadata.len();

    let mime_type = from_path(file_path).first_or_octet_stream().to_string();

    let mut buffer = Vec::new();
    let mut file = File::open(file_path)?;
    std::io::copy(&mut file, &mut buffer)?;

    bucket.put_object_with_content_type(object_name, &buffer, mime_type.as_str()).await?;

    println!("文件 '{}' 已成功上传到 '{}/{}'", file_path, s3_config.bucket_name, object_name);

    Ok(())
}


pub async fn upload_avatar_handler<R: UserRepository>(
    MultipartForm(form): MultipartForm<UploadForm>,
    user_id: web::Path<String>,
    activity_service: web::Data<UserService<R>>,
    s3_config: web::Data<S3Config>,
) -> Result<HttpResponse, Error> {
    let userId = user_id.into_inner();
    // 临时文件路径
    let temp_file_path = format!("./uploads/{}.jpg", userId); // 确保 uploads 目录存在
    let s3_object_name = format!("avatars/{}.jpg", userId);

    // 创建 uploads 目录（如果不存在）
    fs::create_dir_all("./uploads").expect("Failed to create uploads directory");

    // 从 UploadForm 中获取文件名和文件数据
    let filename = form.file.file_name.clone().unwrap_or_else(|| format!("{}.jpg", userId)); // 默认文件名
    let filepath = format!("./uploads/{}", filename);

    // 将文件从 TempFile 移动到指定的路径
    form.file.file.persist(filepath.clone()).map_err(|e| {
        eprintln!("无法将临时文件保存到磁盘: {}", e);
        actix_web::error::ErrorInternalServerError("Failed to save uploaded file")
    })?;

    // 上传文件到 S3
    upload_to_s3(
        &s3_config,
        &filepath,
        &s3_object_name,
    )
        .await
        .map_err(|e| {
            eprintln!("S3 上传错误: {}", e);
            actix_web::error::ErrorInternalServerError("Failed to upload to S3")
        })?;

    // 读取文件并转换为 Base64
    let mut buffer = Vec::new();
    let mut file = File::open(filepath.clone()).map_err(|e| {
        eprintln!("读取文件错误: {}", e);
        actix_web::error::ErrorInternalServerError("Failed to read uploaded file")
    })?;
    std::io::copy(&mut file, &mut buffer).map_err(|e| {
        eprintln!("复制文件到 buffer 错误: {}", e);
        actix_web::error::ErrorInternalServerError("Failed to read uploaded file")
    })?;
    let base64_image = general_purpose::STANDARD.encode(&buffer);

    // 更新数据库
    activity_service
        .update_user_avatar(&userId, &base64_image)
        .await
        .map_err(|e| {
            eprintln!("数据库更新错误: {}", e);
            actix_web::error::ErrorInternalServerError("Failed to update database")
        })?;
    //删除临时文件
    if fs::remove_file(filepath.clone()).is_ok() {
        println!("临时文件 {} 删除成功", filepath);
    } else {
        eprintln!("无法删除临时文件 {}", filepath);
    }
    Ok(HttpResponse::Ok().body("Avatar uploaded and updated successfully"))
}

pub async fn get_user_handler<R: UserRepository>(
    id: web::Path<String>,
    activity_service: web::Data<UserService<R>>,
) -> impl Responder {
    let user_id = id.into_inner();
    match activity_service.get_user(&user_id).await {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().body("User not found"),
        Err(e) => {
            eprintln!("Error: {}", e);
            HttpResponse::InternalServerError().body("Internal Server Error")
        }
    }
}