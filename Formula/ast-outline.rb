class AstOutline < Formula
  desc "Fast, AST-based structural outline for source files"
  homepage "https://github.com/aeroxy/ast-outline"
  url "https://github.com/aeroxy/ast-outline/releases/download/0.4.1/ast-outline-macos-arm64.zip"
  sha256 "a7552ab3f38298101993ad26e2d4e93575c9573f9270fa631b8d0d486b2d834d"
  license "MIT"

  def install
    bin.install "ast-outline"
  end

  test do
    # Run the help command to ensure the binary is functional
    assert_match "Usage: ast-outline", shell_output("#{bin}/ast-outline --help")
  end
end
