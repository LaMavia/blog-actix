use actix_web::HttpResponse;

#[get("/")]
pub fn route() -> HttpResponse {
  HttpResponse::Ok().body(String::from(
    "<!DOCTYPE html>
      <html lang=\"en\">
      
      <head>
        <meta charset=\"UTF-8\">
        <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
        <meta http-equiv=\"X-UA-Compatible\" content=\"ie=edge\">
        <title>Document</title>
        <link rel=\"stylesheet\" href=\"/static/css/styles.css\">
      </head>
      <body>
      <div class=\"root\" id=\"root\"></div>
      </body>
      <script src=\"/js/Index.js\"></script>
      </html>",
  ))
}
