import React from 'react';
import { DollarSignIcon, TrendingUpIcon } from 'lucide-react';
export function BudgetSection() {
  return <div className="bg-white border border-gray-200 rounded-lg p-5">
      <div className="flex justify-between items-center mb-4">
        <h2 className="text-lg font-medium">Budget Tracker</h2>
        <span className="text-sm text-gray-500">Japan Trip</span>
      </div>
      <div className="flex justify-between items-center mb-4">
        <div>
          <p className="text-gray-500 text-sm">Total Budget</p>
          <p className="text-xl font-semibold">$3,500</p>
        </div>
        <div>
          <p className="text-gray-500 text-sm">Spent</p>
          <p className="text-xl font-semibold">$2,180</p>
        </div>
        <div>
          <p className="text-gray-500 text-sm">Remaining</p>
          <p className="text-xl font-semibold text-green-600">$1,320</p>
        </div>
      </div>
      <div className="mb-4">
        <div className="flex justify-between text-xs mb-1">
          <span className="text-gray-600">Budget used</span>
          <span className="font-medium">62%</span>
        </div>
        <div className="w-full bg-gray-100 rounded-full h-2">
          <div className="bg-blue-600 h-2 rounded-full" style={{
          width: '62%'
        }}></div>
        </div>
      </div>
      <div className="space-y-3 mb-4">
        <div className="flex justify-between items-center p-2 bg-gray-50 rounded">
          <div className="flex items-center gap-2">
            <div className="w-8 h-8 rounded-full bg-blue-100 flex items-center justify-center">
              <TrendingUpIcon className="h-4 w-4 text-blue-600" />
            </div>
            <span className="text-sm">Accommodation</span>
          </div>
          <div className="text-right">
            <p className="text-sm font-medium">$1,200</p>
            <p className="text-xs text-gray-500">34%</p>
          </div>
        </div>
        <div className="f lex justify-between items-center p-2 bg-gray-50 rounded">
          <div className="flex items-center gap-2">
            <div className="w-8 h-8 rounded-full bg-green-100 flex items-center justify-center">
              <DollarSignIcon className="h-4 w-4 text-green-600" />
            </div>
            <span className="text-sm">Transportation</span>
          </div>
          <div className="text-right">
            <p className="text-sm font-medium">$650</p>
            <p className="text-xs text-gray-500">19%</p>
          </div>
        </div>
      </div>
      <button className="w-full text-sm text-blue-600 hover:text-blue-700 font-medium">
        View full budget
      </button>
    </div>;
}