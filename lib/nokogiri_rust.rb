require "nokogiri_rust/version"
require "ffi"

module NokogiriRust
  class Node < FFI::AutoPointer
    def self.release(ptr)
      Binding.free(ptr)
    end

    def text
      Binding.text(self)
    end

    module Binding
      extend FFI::Library
      ffi_lib "target/release/libnokogiri_rust.#{FFI::Platform::LIBSUFFIX}"

      attach_function :free, :nokogiri_rust_element_ref_free,
        [Node], :void

      attach_function :text, :nokogiri_rust_element_ref_text,
        [Node], :string
    end
  end

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
      ffi_lib "target/release/libnokogiri_rust.#{FFI::Platform::LIBSUFFIX}"

      attach_function :free, :nokogiri_rust_html_free,
        [HTML], :void
      attach_function :parse, :nokogiri_rust_html_parse,
        [:string], HTML
      attach_function :at_css, :nokogiri_rust_html_at_css,
        [HTML, :string], Node
    end
  end
end

# big_shopping_html = File.read(File.expand_path("big_shopping.html", __dir__))
# selector = ".eIuuYe a, a.EI11Pd, a.AGVhpb, a.GyDBsd, a.VQN8fd, a.VZTCjd, a.REX1ub, a.sHaywe"
# document = NokogiriRust::HTML.parse(big_shopping_html)
# title = document.at_css(selector).text

# puts title
# => "Roblox Game eCard [Digital Download]"
