# frozen_string_literal: true

require 'advent_of_code_2020/day09/problem'

RSpec.describe AdventOfCode2020::Day09::Problem do
  subject(:problem) { described_class.new(fixture_path, preamble: 5) }

  let(:fixture_path) { 'spec/fixtures/day09.txt' }

  describe '#part_one' do
    subject(:part_one) { problem.part_one }

    it { is_expected.to eq(127) }
  end
end
