require "bundler/gem_tasks"
require "rspec/core/rake_task"
require "thermite/tasks"

RSpec::Core::RakeTask.new(:spec)
Thermite::Tasks.new

desc "Benchmark Nokogiri against NokogiriRust"
task "benchmark" do
  ruby "benchmarks/nokogiri_benchmark.rb"
end

desc "Build Rust library in debug mode"
task "thermite:debug" do
  sh "cargo rustc --lib -- -C link-args=-L/home/ilyazub/.rbenv/versions/2.6.3/lib"
end

task default: "thermite:build"
