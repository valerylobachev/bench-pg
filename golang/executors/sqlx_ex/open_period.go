package sqlx_ex

import (
	"bench-pg-go/model/domain"
	"github.com/jmoiron/sqlx"
)

func openPeriod(db *sqlx.DB, period domain.Period, user domain.User) {

	userId := user.Id()

	var materials []string
	err := db.Select(&materials, `select id from fin_materials`)
	if err != nil {
		panic(err)
	}

	for _, materialId := range materials {

		var cnt int64
		err = db.Get(
			&cnt,
			`select count(*) from fin_material_periods 
                   where material_id = $1 and period = $2;`,
			materialId, period.YearPeriod(),
		)
		if err != nil {
			panic(err)
		}

		if cnt == 0 {
			_, err = db.Exec(
				`insert into fin_material_periods
			                select material_id, $1,
			                       (select next_std_price from fin_materials where id = $2) as std_price,
			                       mov_avg_price, actual_price, sell_price, stock,
			                       $3 as updated_by, now() as updated_at
			                from fin_material_periods
			                where material_id = $2 and period = $4`,
				period.YearPeriod(),
				materialId,
				userId,
				period.PrevPeriod().YearPeriod(),
			)
			if err != nil {
				panic(err)
			}
		} else {
			_, err = db.Exec(
				`update fin_material_periods set
						  std_price = (select next_std_price from fin_materials where id = $1),
						  stock = (select stock from fin_material_periods where material_id = $1 and period = $2 ),
						  updated_by = $3, 
						  updated_at = now() 
					   where material_id = $1 and period = $4;`,
				materialId,
				period.PrevPeriod().YearPeriod(),
				userId,
				period.YearPeriod(),
			)
			if err != nil {
				panic(err)
			}

		}

	}
}
