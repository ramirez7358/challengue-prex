use serde::Serialize;

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[derive(Serialize, Debug)]
pub struct SingleResponse<T>
where
    T: Serialize,
{
    pub status: String,
    pub data: T,
}

#[derive(Serialize, Debug)]
pub struct CreateClientResponse {
    pub id: String,
}
