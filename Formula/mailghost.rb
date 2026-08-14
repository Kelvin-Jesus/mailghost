class Mailghost < Formula
  desc "Friendly disposable Mail.tm inbox for your terminal"
  homepage "https://github.com/Kelvin-Jesus/mailghost"
  version "1.1.3"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/Kelvin-Jesus/mailghost/releases/download/v1.1.3/mailghost-v1.1.3-macos-aarch64.tar.gz"
      sha256 "64c5c930dff50ddcb5fa2d90ee177f6f7672744c810008b85632b9e92e077f8c"
    end
    on_intel do
      url "https://github.com/Kelvin-Jesus/mailghost/releases/download/v1.1.3/mailghost-v1.1.3-macos-x86_64.tar.gz"
      sha256 "1d16881c33b660c012b00c83ed92482746c42b64962d5697f9d25d1f4552b9c3"
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/Kelvin-Jesus/mailghost/releases/download/v1.1.3/mailghost-v1.1.3-linux-aarch64.tar.gz"
      sha256 "47c069722d4de9f07b91e8479616af304ea4127dac53c6cc64977fe96d9395b2"
    end
    on_intel do
      url "https://github.com/Kelvin-Jesus/mailghost/releases/download/v1.1.3/mailghost-v1.1.3-linux-x86_64.tar.gz"
      sha256 "8e5bfb939837eddd0addd7e042650d6a458ccc3e40f1ad3b540f926a139194af"
    end
  end

  def install
    bin.install "mailghost"
    doc.install "README.md"
    pkgshare.install "LICENSE"
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/mailghost --version")
  end
end
