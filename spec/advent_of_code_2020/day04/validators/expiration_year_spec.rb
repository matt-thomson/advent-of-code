# frozen_string_literal: true

require 'advent_of_code_2020/day04/validators/expiration_year'

RSpec.describe AdventOfCode2020::Day04::Validators::ExpirationYear do
  subject(:validator) { described_class.new }

  describe '#valid?' do
    subject(:valid?) { validator.valid?(input) }

    context 'with a valid input' do
      let(:input) { '2025' }

      it { is_expected.to eq(true) }
    end

    context 'with a year that is too late' do
      let(:input) { '2031' }

      it { is_expected.to eq(false) }
    end

    context 'with a year that is too early' do
      let(:input) { '2019' }

      it { is_expected.to eq(false) }
    end

    context 'with non-digits' do
      let(:input) { 'abcd' }

      it { is_expected.to eq(false) }
    end
  end
end
