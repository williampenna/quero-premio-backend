CREATE TABLE usuario
(
    id               SERIAL PRIMARY KEY,
    nome             VARCHAR NOT NULL,
    cpf              VARCHAR NOT NULL,
    data_criacao     TIMESTAMP NOT NULL,
    data_atualizacao TIMESTAMP NOT NULL,
    data_nascimento  DATE NOT NULL
)