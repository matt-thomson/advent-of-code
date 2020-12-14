# frozen_string_literal: true

require 'advent_of_code_2020/day14/mask'

RSpec.describe AdventOfCode2020::Day14::Mask do
  subject(:mask) { described_class.new('XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X') }

  describe '#apply' do
    subject(:apply) { mask.apply(input) }

    context 'with first example' do
      let(:input) { 11 }

      it { is_expected.to eq(73) }
    end

    context 'with second example' do
      let(:input) { 101 }

      it { is_expected.to eq(101) }
    end

    context 'with third example' do
      let(:input) { 0 }

      it { is_expected.to eq(64) }
    end
  end
end
