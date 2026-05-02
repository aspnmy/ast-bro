class AstOutline < Formula
  desc "Fast, AST-based structural outline for source files"
  homepage "https://github.com/aeroxy/ast-outline"
  url "https://github.com/aeroxy/ast-outline/releases/download/0.4.0/ast-outline-macos-arm64.zip"
  sha256 "6ca6eb2275de012ad87ee0ca42eb963bf0f22b6ee7ab8a7658d8f0e71a42f4bc"
  license "MIT"

  def install
    bin.install "ast-outline"
  end

  test do
    # Run the help command to ensure the binary is functional
    assert_match "Usage: ast-outline", shell_output("#{bin}/ast-outline --help")
  end
end
