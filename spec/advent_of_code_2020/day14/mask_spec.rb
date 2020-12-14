# frozen_string_literal: true

require 'advent_of_code_2020/day14/mask'

RSpec.describe AdventOfCode2020::Day14::Mask do
  subject(:mask) { described_class.new(raw_mask) }

  describe '#apply_v1' do
    subject(:apply_v1) { mask.apply_v1(input) }

    let(:raw_mask) { 'XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X' }

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

  describe '#apply_v2' do
    subject(:apply_v2) { mask.apply_v2(input) }

    context 'with first example' do
      let(:raw_mask) { '000000000000000000000000000000X1001X' }
      let(:input) { 42 }

      it { is_expected.to contain_exactly(26, 27, 58, 59) }
    end

    context 'with second example' do
      let(:raw_mask) { '00000000000000000000000000000000X0XX' }
      let(:input) { 26 }

      it { is_expected.to contain_exactly(16, 17, 18, 19, 24, 25, 26, 27) }
    end
  end
end
