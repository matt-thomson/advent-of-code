# frozen_string_literal: true

require 'advent_of_code_2020/day12/problem'

RSpec.describe AdventOfCode2020::Day12::Problem do
  subject(:problem) { described_class.new(fixture_path) }

  let(:fixture_path) { 'spec/fixtures/day12.txt' }

  describe '#part_one' do
    subject(:part_one) { problem.part_one }

    it { is_expected.to eq(25) }
  end

  describe '#part_tw0' do
    subject(:part_one) { problem.part_two }

    it { is_expected.to eq(286) }
  end
end
