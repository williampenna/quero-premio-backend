// @generated automatically by Diesel CLI.

diesel::table! {
    usuario (id) {
        id -> Int4,
        nome -> Varchar,
        cpf -> Varchar,
        data_criacao -> Timestamptz,
        data_atualizacao -> Timestamptz,
        data_nascimento -> Date,
    }
}
