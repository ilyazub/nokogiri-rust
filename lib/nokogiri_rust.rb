require "nokogiri_rust/version"
require "ffi"

module NokogiriRust
  class HTML < FFI::AutoPointer
    def self.release(ptr)
      Binding.free(ptr)
    end

    def self.parse(html)
      Binding.parse(html)
    end

    def at_css(selector)
      Binding.at_css(self, selector)
    end

    module Binding
      extend FFI::Library
      ffi_lib "target/debug/libnokogiri_rust.#{FFI::Platform::LIBSUFFIX}"

      attach_function :free, :nokogiri_rust_free,
        [HTML], :void
      attach_function :parse, :nokogiri_rust_parse,
        [:string], HTML
      attach_function :at_css, :nokogiri_rust_at_css,
        [HTML, :string], :string
    end
  end
end

# big_shopping_html = File.read(File.expand_path("big_shopping.html", __dir__)).freeze; selector = ".eIuuYe a, a.EI11Pd, a.AGVhpb, a.GyDBsd, a.VQN8fd, a.VZTCjd, a.REX1ub, a.sHaywe".freeze; document = NokogiriRust::HTML.parse(big_shopping_html)
# title = document.at_css(selector)

# puts title
