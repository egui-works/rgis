pub struct LoadFileJob<F: crate::FileLoader> {
    pub file_loader: F,
    pub name: String,
    pub source_crs: String,
}

pub struct LoadFileJobOutcome {
    pub geometry: geo_features::FeatureCollection,
    pub name: String,
    pub source_crs: String,
}

impl<F: crate::FileLoader + Sync + Send + 'static> bevy_jobs::Job for LoadFileJob<F>
where
    <F as crate::FileLoader>::Error: Send + Sync + 'static,
{
    type Outcome = Result<LoadFileJobOutcome, F::Error>;

    fn name(&self) -> String {
        "Loading FIXME FIXME GeoJson file".into()
    }

    fn perform(self) -> bevy_jobs::AsyncReturn<Self::Outcome> {
        Box::pin(async move {
            Ok(LoadFileJobOutcome {
                geometry: self.file_loader.load()?,
                name: self.name,
                source_crs: self.source_crs,
            })
        })
    }
}