use super::{outcome::SourceError, prepare::Bounds};

pub fn validate_count(count: usize, bounds: Bounds) -> Result<(), SourceError> {
    if bounds.min > bounds.max {
        return Err(SourceError::Internal(
            "limites de contagem invertidos".into(),
        ));
    }
    if !(bounds.min..=bounds.max).contains(&count) {
        return Err(SourceError::Empty(format!(
            "{count} registros fora do intervalo {}–{}",
            bounds.min, bounds.max
        )));
    }
    Ok(())
}

pub fn validate_shrink(
    previous: usize,
    removed: u64,
    allow_shrink: bool,
) -> Result<(), SourceError> {
    if !allow_shrink && removed > (previous / 5) as u64 {
        return Err(SourceError::Empty(format!(
            "suspicious shrink: {removed} de {previous} registros removidos"
        )));
    }
    Ok(())
}
