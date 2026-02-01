#[cfg(test)]
mod tests {
    use freki::utils::DataDeletionError;
    use freki::vector_db::VectorDbError;

    #[test]
    fn test_data_deletion_error_display() {
        let e = DataDeletionError::VectorDb(VectorDbError::VectorError("test".to_string()));
        assert!(e.to_string().contains("Vector DB"));
    }

    #[test]
    fn test_data_deletion_error_from_vector_db_error() {
        let v = VectorDbError::ConnectionError("conn".to_string());
        let e: DataDeletionError = v.into();
        assert!(e.to_string().contains("conn"));
    }
}
