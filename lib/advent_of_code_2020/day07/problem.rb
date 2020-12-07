# frozen_string_literal: true

require_relative 'rule'

module AdventOfCode2020
  module Day07
    class Problem
      def initialize(path)
        @rules = File.readlines(path).map { |line| Rule.parse(line) }
      end

      def part_one
        allowed = Set.new
        to_check = ['shiny gold']

        until to_check.empty?
          colour = to_check.shift

          containers = @rules.select { |rule| rule.conditions.include?(colour) }.map(&:colour)
          new_containers = containers.reject { |container| allowed.include?(container) }

          allowed += new_containers
          to_check += new_containers
        end

        allowed.count
      end
    end
  end
end
