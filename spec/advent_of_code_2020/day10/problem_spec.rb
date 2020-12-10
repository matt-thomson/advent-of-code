# frozen_string_literal: true

require 'advent_of_code_2020/day10/problem'

RSpec.describe AdventOfCode2020::Day10::Problem do
  subject(:problem) { described_class.new(fixture_path) }

  describe '#part_one' do
    subject(:part_one) { problem.part_one }

    context 'with first example' do
      let(:fixture_path) { 'spec/fixtures/day10a.txt' }

      it { is_expected.to eq(35) }
    end

    context 'with second example' do
      let(:fixture_path) { 'spec/fixtures/day10b.txt' }

      it { is_expected.to eq(220) }
    end
  end
end
