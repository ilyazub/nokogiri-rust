require_relative "lib/nokogiri_rust/version"

Gem::Specification.new do |spec|
  spec.name = "nokogiri_rust"
  spec.version = NokogiriRust::VERSION
  spec.authors = ["Ilya Zub"]
  spec.email = ["zaoooza92@gmail.com"]

  spec.summary = "Nokogiri in Rust"
  spec.homepage = "https://github.com/ilyazub/nokogiri-rust"
  spec.license = "MIT OR Apache-2.0"

  spec.metadata["homepage_uri"] = spec.homepage

  spec.add_runtime_dependency "fiddle"

  spec.add_development_dependency "pry", "~> 0.14"
  spec.add_development_dependency "benchmark-ips"
  spec.add_development_dependency "nokogiri"

  # Specify which files should be added to the gem when it is released.
  # The `git ls-files -z` loads the files in the RubyGem that have been added into git.
  spec.files = Dir.chdir(File.expand_path("..", __FILE__)) do
    `git ls-files -z`.split("\x0").reject { |f| f.match(%r{^(test|spec|features)/}) }
  end

  spec.bindir = "exe"
  spec.executables = spec.files.grep(%r{^exe/}) { |f| File.basename(f) }
  spec.require_paths = ["lib"]
end
