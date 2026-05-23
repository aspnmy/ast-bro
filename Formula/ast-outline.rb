# Final ast-outline release (2.1.1) — installs ast-outline and the new
# ast-bro / sb commands together so existing users can transition.
# New versions ship as ast-bro.rb.
class AstOutline < Formula
  desc "Final ast-outline release; renamed to ast-bro. Ships ast-bro and sb alongside."
  homepage "https://github.com/aeroxy/ast-bro"
  url "https://github.com/aeroxy/ast-bro/releases/download/2.1.1/ast-outline-macos-arm64.zip"
  sha256 "e153fc2a65c41ea35e1dd0e7e87ca3d4ec914be6614bbcb5d12cfbe473cc2089"
  license "MIT"

  def install
    bin.install "ast-outline"
    bin.install_symlink "ast-outline" => "ast-bro"
    bin.install_symlink "ast-outline" => "sb"
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/ast-outline --version")
  end
end
