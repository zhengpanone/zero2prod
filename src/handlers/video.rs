// use hyper::{ Response, header, StatusCode};
// use tokio::io::BufReader;
// use tokio_util::io::ReaderStream;
// use bytes::Bytes;
// use crate::utils::custom_response::ApiResponse;
// use crate::handlers::handlers::ApiError;
// use hyper::Error as HyperError;
// use tokio::fs::File;

// pub async fn stream_video() -> Result<ApiResponse<String>, ApiError> {
//     let file_path = "/app/mp4_server/test.mp4"; // MP4 文件路径

//     // 打开文件
//     let file = match File::open(file_path).await {
//         Ok(file) => file,
//         Err(_) => {
//             return Ok(ApiResponse::<String>::new(
//                 Some("File not found".into()), 
//                 None, 
//                 StatusCode::NOT_FOUND,
//             ));
//         },
//     };

//     // 获取文件大小
//     let metadata = match file.metadata().await {
//         Ok(metadata) => metadata,
//         Err(_) => {
//             return Ok(ApiResponse::<String>::new(
//                 Some("Unable to get file size".into()), 
//                 None, 
//                 StatusCode::INTERNAL_SERVER_ERROR,
//             ));
//         },
//     };
//     let file_size = metadata.len();

//     // 将文件转换为流
//     let reader = BufReader::new(file);
//     let reader_stream = ReaderStream::new(reader);
    
//   // 构建响应体
//   let response_body = reader_stream.map(|result| {
//     match result {
//         Ok(bytes) => Ok(Bytes::from(bytes)),
//         Err(e) => {
//             eprintln!("Error reading from file: {}", e);
//             Err(HyperError::from(std::io::Error::new(std::io::ErrorKind::Other, "Stream error")))
//         }
//     }
// });

//     // 返回流响应
//     let mut response = Response::new(response_body);
//     response.headers_mut().insert(header::CONTENT_LENGTH, header::HeaderValue::try_from(file_size.to_string()).unwrap());
//     response.headers_mut().insert(hyper::header::CONTENT_TYPE, "video/mp4".parse().unwrap());

//     // 返回成功响应
//     Ok(ApiResponse::<String>::new(
//         None, 
//         Some("Video streaming started".into()), 
//         StatusCode::OK,
//     ))
// }