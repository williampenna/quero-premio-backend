// @generated automatically by Diesel CLI.

diesel::table! {
    usuario (id) {
        id -> Int4,
        nome -> Varchar,
        cpf -> Varchar,
        data_criacao -> Timestamp,
        data_atualizacao -> Timestamp,
        data_nascimento -> Date,
    }
}
