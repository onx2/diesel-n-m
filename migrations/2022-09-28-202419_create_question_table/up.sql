CREATE TABLE question (
    id UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
    text TEXT NOT NULL,
    input_type TEXT NOT NULL,
    answer_options JSONB NOT NULL,
    advanced_options JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX question_id_idx ON question (id);

SELECT diesel_manage_updated_at('question');