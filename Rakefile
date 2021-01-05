require "bundler/gem_tasks"
require "rspec/core/rake_task"

RSpec::Core::RakeTask.new(:spec)

desc "Benchmark Nokogiri against NokogiriRust"
task "benchmark" do
  ruby "benchmarks/nokogiri_benchmark.rb"
end

namespace :extension do
  task :clean do
    sh "cargo clean"
  end

  namespace :build do
    desc "Build Rust library in debug mode"
    task :debug do
      sh "cargo rustc --lib -- -C link-args=-L/home/ilyazub/.rbenv/versions/2.7.2/lib"
    end

    desc "Build Rust library in debug mode"
    task :release do
      sh "cargo rustc --release --lib -- -C link-args=-L/home/ilyazub/.rbenv/versions/2.7.2/lib"
    end
  end
end

task default: "extension:build:release"
