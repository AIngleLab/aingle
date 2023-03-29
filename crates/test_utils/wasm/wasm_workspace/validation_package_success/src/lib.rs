use adk::prelude::*;

const NUM_SONGS: usize = 30;

#[adk_entry(id = "song", required_validation_type = "element")]
struct Song;

#[adk_entry(id = "artist", required_validation_type = "custom")]
struct Artist;

entry_defs![Song::entry_def(), Artist::entry_def()];

#[adk_extern]
fn validation_package(
    app_entry_type: AppEntryType,
) -> ExternResult<ValidationPackageCallbackResult> {
    let index = app_entry_type.id();
    match u8::from(index) {
        // Artist
        1 => {
            let query = QueryFilter::new()
                .entry_type(EntryType::App(AppEntryType::new(
                    0.into(),
                    0.into(),
                    EntryVisibility::Public,
                )))
                .include_entries(true);
            let songs = adk::prelude::query(query)?;
            // Need to post at least 30 songs to be an artist on this sgd
            if songs.len() >= NUM_SONGS {
                Ok(ValidationPackageCallbackResult::Success(
                    ValidationPackage::new(songs),
                ))
            } else {
                Ok(ValidationPackageCallbackResult::Fail(
                    "Need at least 30 songs to be an artist on this sgd".to_string(),
                ))
            }
        }
        _ => Ok(ValidationPackageCallbackResult::Success(
            ValidationPackage::new(vec![]),
        )),
    }
}

#[adk_extern]
fn commit_artist(_: ()) -> ExternResult<HeaderHash> {
    create_entry(&Artist)
}

#[adk_extern]
fn commit_songs(_: ()) -> ExternResult<()> {
    for _ in 0..NUM_SONGS {
        create_entry(&Song)?;
    }
    Ok(())
}
