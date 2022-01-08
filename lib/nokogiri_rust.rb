require 'nokogiri_rust/version'
# require "fiddle"
require 'rutie'

# Fiddle::Function.new(Fiddle.dlopen(NokogiriRust.lib_path)["Init_nokogiri_rust"], [], Fiddle::TYPE_VOIDP).call

module NokogiriRust
  Rutie.new(:nokogiri_rust_ruby_lib).init 'Init_nokogiri_rust', __dir__
end

# module NokogiriRust
#   def self.operating_system
#     case RbConfig::CONFIG["host_os"].downcase
#     when /linux|bsd|solaris/ then "linux"
#     when /darwin/ then "macos"
#     when /mingw|mswin/ then "windows"
#     else "unknown"
#     end
#   end

#   def self.lib_path
#     File.join(__dir__, [operating_system, "-ruby", RUBY_VERSION.split(".")[0...-1].join(".")].join(""))
#   end
# end

# big_shopping_html = File.read(File.expand_path("big_shopping.html", __dir__))
# selector = ".eIuuYe a, a.EI11Pd, a.AGVhpb, a.GyDBsd, a.VQN8fd, a.VZTCjd, a.REX1ub, a.sHaywe"
# document = NokogiriRust::HTML.parse(big_shopping_html)
# title = document.at_css(selector).text

# puts title
# => "Roblox Game eCard [Digital Download]"
