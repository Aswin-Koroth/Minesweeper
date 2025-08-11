fn main() {
    #[cfg(target_os = "windows")]
    {
        let mut res = winres::WindowsResource::new();

        res.set_icon("icon.ico");
        res.set("FileDescription", "Minesweeper v1.0.1");
        res.set("ProductName", "Minesweeper");
        res.set("ProductVersion", "1.0.1");
        res.set("CompanyName", "Aswin Koroth");
        res.set("LegalCopyright", "Copyright (c) 2025");

        res.compile().unwrap();
    }
}
