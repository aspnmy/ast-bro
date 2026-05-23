class AstBro < Formula
  desc "Fast AST-based code-navigation toolkit: shape, surface, deps, search, pattern matching"
  homepage "https://github.com/aeroxy/ast-bro"
  url "https://github.com/aeroxy/ast-bro/releases/download/2.2.0/ast-bro-macos-arm64.zip"
  sha256 "6c2ddb98fcc804131716bb0352f0c79795779a8f0b126c7651072c8dd5fb4b90"
  license "MIT"

  def install
    bin.install "ast-bro"
    bin.install_symlink "ast-bro" => "ast-outline"
    bin.install_symlink "ast-bro" => "sb"
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/ast-bro --version")
  end
end
