pub fn ids_to_string(ids: Vec<u32>) -> String {
    // Transform the Vec to a comma-separated String
    let mut ids: String = ids.iter().map(|id| format!("{},", id.to_string())).collect();
    ids.pop(); // Remove the last comma
    ids
}
