CREATE TABLE usuario
(
    id               SERIAL PRIMARY KEY,
    nome             VARCHAR NOT NULL,
    cpf              VARCHAR NOT NULL,
    data_criacao     DATETIME NOT NULL,
    data_atualizacao DATETIME NOT NULL,
    data_nascimento  DATE NOT NULL,
)