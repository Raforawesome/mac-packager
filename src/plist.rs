pub fn create(bundle_name: String, display_name: String, bundle_id: String, sig: String, app_name: String) -> String {
	format!(r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
	<key>CFBundleName</key>
	<string>{}</string>
	<key>CFBundleDisplayName</key>
	<string>{}</string>
	<key>CFBundleIdentifier</key>
	<string>{}</string>
	<key>CFBundleVersion</key>
	<string>0.0.1</string>
	<key>CFBundlePackageType</key>
	<string>APPL</string>
	<key>CFBundleSignature</key>
	<string>{}</string>
	<key>CFBundleExecutable</key>
	<string>{}</string>
</dict>
</plist>
"#, bundle_name, display_name, bundle_id, sig, app_name)
}