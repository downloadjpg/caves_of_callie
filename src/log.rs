trait AnnounceExt {
    fn announce(&mut self, text: impl Into<String>);
}
impl AnnounceExt for Commands<'_, '_> {
    fn announce(&mut self, text: impl Into<String>) {
        let text = text.into();
        self.queue(move |world: &mut World| {
            world
                .resource_mut::<AnnouncementQueue>()
                .pending
                .push_back(text);
        });
    }
}
