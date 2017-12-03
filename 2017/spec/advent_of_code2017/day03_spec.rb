# frozen_string_literal: true

require 'advent_of_code2017/day03'

RSpec.describe AdventOfCode2017::Day03 do
  describe '.solve_part_one' do
    subject { described_class.solve_part_one(input) }

    context 'with example 1' do
      let(:input) { 1 }

      it { is_expected.to eq(0) }
    end

    context 'with example 2' do
      let(:input) { 12 }

      it { is_expected.to eq(3) }
    end

    context 'with example 3' do
      let(:input) { 23 }

      it { is_expected.to eq(2) }
    end

    context 'with example 4' do
      let(:input) { 1024 }

      it { is_expected.to eq(31) }
    end
  end
end
