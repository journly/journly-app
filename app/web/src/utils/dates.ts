export const formatTripDatesDisplay = (dates: [string | null, string | null]) => {
  if (dates[0] && dates[1] && dates[0] === dates[1]) {
    return `${new Date(dates[0]).toLocaleDateString('en-US', {
      month: 'short',
      day: 'numeric',
      year: 'numeric',
    })}`;
  }
  if (dates[0] && dates[1]) {
    return `${new Date(dates[0]).toLocaleDateString('en-US', {
      month: 'short',
      day: 'numeric',
      year: 'numeric',
    })} - ${new Date(dates[1]).toLocaleDateString('en-US', {
      month: 'short',
      day: 'numeric',
      year: 'numeric',
    })}`;
  }
  if (dates[0]) {
    return `${new Date(dates[0]).toLocaleDateString('en-US', {
      month: 'short',
      day: 'numeric',
      year: 'numeric',
    })}`;
  }
  return 'TBD';
};

export const formatTripDatesSimple = (dates: [string | null, string | null]) => {
  if (dates[0] && dates[1] && dates[0] === dates[1]) {
    return new Date(dates[0]).toLocaleDateString();
  }
  if (dates[0] && dates[1]) {
    return `${new Date(dates[0]).toLocaleDateString()} - ${new Date(dates[1]).toLocaleDateString()}`;
  }
  return null;
};

function getDaysBetween(date1: Date, date2: Date) {
  const oneDay = 24 * 60 * 60 * 1000; // milliseconds in a day
  const diffTime = Math.abs(date2.getTime() - date1.getTime());
  return Math.ceil(diffTime / oneDay);
}

export const getTripDurationString = (dates: [string | undefined, string | undefined]) => {
  if (dates[0] && dates[1] && dates[0] === dates[1]) {
    return '1 day';
  }
  if (dates[0] && dates[1]) {
    return `${getDaysBetween(new Date(dates[0]), new Date(dates[1])) + 1} days`;
  }
  return 'N/A';
};
