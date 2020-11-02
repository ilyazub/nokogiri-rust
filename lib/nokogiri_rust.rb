require "nokogiri_rust/version"
require "rutie"
require "thermite/config"
require "fiddle"
require "fiddle/import"

module NokogiriRust
  FFI_LIBRARY = begin
    toplevel_dir = File.dirname(__dir__)

    config = Thermite::Config.new(
      cargo_project_path: toplevel_dir,
      ruby_project_path: toplevel_dir
    )

    config.ruby_extension_path
  end

  Fiddle::Function.new(Fiddle.dlopen(FFI_LIBRARY)["Init_nokogiri_rust"], [], Fiddle::TYPE_VOIDP).call
end
