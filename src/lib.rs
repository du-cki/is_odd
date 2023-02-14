use serde_json::json;
use worker::*;

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    let router = Router::new();

    router
        .get_async("/:number", |_, ctx| async move {
            if let Some(number) = ctx.param("number") {
                if let Ok(num) = number.parse::<i32>() {
                    return Response::from_json(&json!({ "number": num, "is_odd": num % 2 != 0 }))
                }
            }

            Response::error("Bad Request", 400)
        })
        .run(req, env)
        .await
}
