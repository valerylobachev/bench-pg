package domain

import (
	"cloud.google.com/go/civil"
	"fmt"
	"time"
)

type Period struct {
	year  int
	month int
}

func NewPeriod(year int, month int) Period {
	return Period{year, month}
}

func PeriodFromDate(date civil.Date) Period {
	return Period{date.Year, int(date.Month)}
}

func (p Period) String() string {
	return fmt.Sprintf("%04d.%02d", p.year, p.month)
}

func (p Period) YearPeriod() int {
	return p.year*100 + p.month
}
func (p Period) NextPeriod() Period {
	if p.month == 12 {
		return NewPeriod(p.year+1, 1)
	}
	return NewPeriod(p.year, p.month+1)
}

func (p Period) PrevPeriod() Period {
	if p.month == 1 {
		return NewPeriod(p.year-1, 12)
	}
	return NewPeriod(p.year, p.month-1)
}

func (p Period) IsLeapYear() bool {
	year := p.year
	return year%4 == 0 && (year%100 != 0 || year%400 == 0)
}

func (p Period) LastDay() int {
	switch p.month {
	case 1, 3, 5, 7, 8, 10, 12:
		return 31
	case 4, 6, 9, 11:
		return 30
	case 2:
		if p.IsLeapYear() {
			return 29
		}
		return 28
	}
	return 0
}

func (p Period) FirstDate() civil.Date {
	return civil.Date{
		Year:  p.year,
		Month: time.Month(p.month),
		Day:   1,
	}
}

func (p Period) LastDate() civil.Date {
	return civil.Date{
		Year:  p.year,
		Month: time.Month(p.month),
		Day:   p.LastDay(),
	}
}

func (p Period) Date(day int) civil.Date {
	if day < 1 {
		day = 1
	} else if day > p.LastDay() {
		day = p.LastDay()
	}
	return civil.Date{
		Year:  p.year,
		Month: time.Month(p.month),
		Day:   day,
	}
}

func (p Period) FirstPeriod() Period {
	return NewPeriod(p.year, 1)
}
func (p Period) LastPeriod() Period {
	return NewPeriod(p.year, 12)
}

func (p Period) Year() int {
	return p.year
}
func (p Period) Month() int {
	return p.month
}
