CREATE TABLE form_question (
    id UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
    form_id UUID NOT NULL REFERENCES form (id) ON DElETE CASCADE,
    question_id UUID NOT NULL REFERENCES question (id) ON DElETE CASCADE,
    "order" INTEGER NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (form_id, question_id)
);

CREATE UNIQUE INDEX form_question_id_idx ON form_question (id);
CREATE UNIQUE INDEX form_id_question_id_idx ON form_question (form_id, question_id);

SELECT diesel_manage_updated_at('form_question');