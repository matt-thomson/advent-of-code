# frozen_string_literal: true

require 'advent_of_code_2020/day04/validators/issue_year'

RSpec.describe AdventOfCode2020::Day04::Validators::IssueYear do
  subject(:validator) { described_class.new }

  describe '#valid?' do
    subject(:valid?) { validator.valid?(input) }

    context 'with a valid input' do
      let(:input) { '2015' }

      it { is_expected.to be(true) }
    end

    context 'with a year that is too late' do
      let(:input) { '2021' }

      it { is_expected.to be(false) }
    end

    context 'with a year that is too early' do
      let(:input) { '2009' }

      it { is_expected.to be(false) }
    end

    context 'with non-digits' do
      let(:input) { 'abcd' }

      it { is_expected.to be(false) }
    end
  end
end
