use bybe::InitializeLogResponsibility;

fn main() -> std::io::Result<()> {
    bybe::start(None, None, None, InitializeLogResponsibility::Personal)
}
