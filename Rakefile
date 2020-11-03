require "bundler/gem_tasks"
require "rspec/core/rake_task"

RSpec::Core::RakeTask.new(:spec)

desc "Benchmark Nokogiri against NokogiriRust"
task "benchmark" do
  ruby "benchmarks/nokogiri_benchmark.rb"
end

desc "Build Rust library in debug mode"
task "extension:build:debug" do
  sh "cargo rustc --lib -- -C link-args=-L/home/ilyazub/.rbenv/versions/2.6.3/lib"
end

desc "Build Rust library in debug mode"
task "extension:build:release" do
  sh "cargo rustc --release --lib -- -C link-args=-L/home/ilyazub/.rbenv/versions/2.6.3/lib"
end

task default: "extension:build"
