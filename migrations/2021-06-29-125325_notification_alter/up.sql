-- Your SQL goes here
ALTER TABLE notification_ref ADD CONSTRAINT notification_ref_fk0 FOREIGN KEY (id_degree_importance) REFERENCES degree_importance_ref(id);
