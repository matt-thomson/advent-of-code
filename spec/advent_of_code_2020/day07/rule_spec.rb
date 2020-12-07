# frozen_string_literal: true

require 'advent_of_code_2020/day07/rule'

RSpec.describe AdventOfCode2020::Day07::Rule do
  describe '.parse' do
    subject(:rule) { described_class.parse(input) }

    context 'with a single condition' do
      let(:input) { 'bright white bags contain 1 shiny gold bag.' }

      it 'parses the colour' do
        expect(rule.colour).to eq('bright white')
      end

      it 'parses the condition' do
        expect(rule.conditions).to eq('shiny gold' => 1)
      end
    end

    context 'with multiple conditions' do
      let(:input) { 'dark orange bags contain 3 bright white bags, 4 muted yellow bags.' }

      it 'parses the colour' do
        expect(rule.colour).to eq('dark orange')
      end

      it 'parses the conditions' do
        expect(rule.conditions).to eq('bright white' => 3, 'muted yellow' => 4)
      end
    end

    context 'with no conditions' do
      let(:input) { 'dotted black bags contain no other bags.' }

      it 'parses the colour' do
        expect(rule.colour).to eq('dotted black')
      end

      it 'parses the conditions' do
        expect(rule.conditions).to be_empty
      end
    end
  end
end
