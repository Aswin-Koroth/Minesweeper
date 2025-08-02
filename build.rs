fn main() {
    #[cfg(target_os = "windows")]
    {
        let mut res = winres::WindowsResource::new();

        res.set_icon("icon.ico");
        res.set("FileDescription", "Minesweeper Game");
        res.set("ProductName", "Minesweeper");
        res.set("ProductVersion", "0.5.0");
        res.set("CompanyName", "Aswin Koroth");
        res.set("LegalCopyright", "Copyright (c) 2025");

        res.compile().unwrap();
    }
}
