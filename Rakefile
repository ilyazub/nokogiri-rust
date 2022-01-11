require 'bundler/gem_tasks'
require 'rspec/core/rake_task'

RSpec::Core::RakeTask.new(:spec, [] => ['extension:build'])

desc 'Benchmark Nokogiri against NokogiriRust'
task :benchmark do
  ruby 'benchmarks/nokogiri_benchmark.rb'
end

namespace :extension do
  desc 'Clean built files'
  task :clean do
    sh 'cargo clean'
  end

  desc 'Build Rust shared library'
  task :build do
    sh 'cargo build'
  end

  desc 'Build Rust shared library in release mode (for performance)'
  task 'build:release' do
    sh 'cargo build --release'
  end

  task default: ['extension:build']
end

task default: :spec
