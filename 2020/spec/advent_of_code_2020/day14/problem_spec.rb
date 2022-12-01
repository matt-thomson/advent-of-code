# frozen_string_literal: true

require 'advent_of_code_2020/day14/problem'

RSpec.describe AdventOfCode2020::Day14::Problem do
  subject(:problem) { described_class.new(fixture_path) }

  describe '#part_one' do
    subject(:part_one) { problem.part_one }

    let(:fixture_path) { 'spec/fixtures/day14a.txt' }

    it { is_expected.to eq(165) }
  end

  describe '#part_two' do
    subject(:part_one) { problem.part_two }

    let(:fixture_path) { 'spec/fixtures/day14b.txt' }

    it { is_expected.to eq(208) }
  end
end
