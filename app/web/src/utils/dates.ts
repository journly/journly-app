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
