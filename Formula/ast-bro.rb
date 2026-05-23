class AstBro < Formula
  desc "Fast AST-based code-navigation toolkit: shape, surface, deps, search, pattern matching"
  homepage "https://github.com/aeroxy/ast-bro"
  url "https://github.com/aeroxy/ast-bro/releases/download/2.2.0/ast-bro-macos-arm64.zip"
  sha256 "0561d9431b5d9f58aeeb838a8ab980d41850358df1040512b2e148c201df0e80"
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
