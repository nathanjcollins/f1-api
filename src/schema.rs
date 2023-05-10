// @generated automatically by Diesel CLI.

diesel::table! {
    circuits (circuitId) {
        circuitId -> Integer,
        circuitRef -> Varchar,
        name -> Varchar,
        location -> Nullable<Varchar>,
        country -> Nullable<Varchar>,
        lat -> Nullable<Float>,
        lng -> Nullable<Float>,
        alt -> Nullable<Integer>,
        url -> Varchar,
    }
}

diesel::table! {
    constructorResults (constructorResultsId) {
        constructorResultsId -> Integer,
        raceId -> Integer,
        constructorId -> Integer,
        points -> Nullable<Float>,
        status -> Nullable<Varchar>,
    }
}

diesel::table! {
    constructorStandings (constructorStandingsId) {
        constructorStandingsId -> Integer,
        raceId -> Integer,
        constructorId -> Integer,
        points -> Float,
        position -> Nullable<Integer>,
        positionText -> Nullable<Varchar>,
        wins -> Integer,
    }
}

diesel::table! {
    constructors (constructorId) {
        constructorId -> Integer,
        constructorRef -> Varchar,
        name -> Varchar,
        nationality -> Nullable<Varchar>,
        url -> Varchar,
    }
}

diesel::table! {
    driverStandings (driverStandingsId) {
        driverStandingsId -> Integer,
        raceId -> Integer,
        driverId -> Integer,
        points -> Float,
        position -> Nullable<Integer>,
        positionText -> Nullable<Varchar>,
        wins -> Integer,
    }
}

diesel::table! {
    drivers (driverId) {
        driverId -> Integer,
        driverRef -> Varchar,
        number -> Nullable<Integer>,
        code -> Nullable<Varchar>,
        forename -> Varchar,
        surname -> Varchar,
        dob -> Nullable<Date>,
        nationality -> Nullable<Varchar>,
        url -> Varchar,
    }
}

diesel::table! {
    lapTimes (raceId, driverId, lap) {
        raceId -> Integer,
        driverId -> Integer,
        lap -> Integer,
        position -> Nullable<Integer>,
        time -> Nullable<Varchar>,
        milliseconds -> Nullable<Integer>,
    }
}

diesel::table! {
    pitStops (raceId, driverId, stop) {
        raceId -> Integer,
        driverId -> Integer,
        stop -> Integer,
        lap -> Integer,
        time -> Time,
        duration -> Nullable<Varchar>,
        milliseconds -> Nullable<Integer>,
    }
}

diesel::table! {
    qualifying (qualifyId) {
        qualifyId -> Integer,
        raceId -> Integer,
        driverId -> Integer,
        constructorId -> Integer,
        number -> Integer,
        position -> Nullable<Integer>,
        q1 -> Nullable<Varchar>,
        q2 -> Nullable<Varchar>,
        q3 -> Nullable<Varchar>,
    }
}

diesel::table! {
    races (raceId) {
        raceId -> Integer,
        year -> Integer,
        round -> Integer,
        circuitId -> Integer,
        name -> Varchar,
        date -> Date,
        time -> Nullable<Time>,
        url -> Nullable<Varchar>,
        fp1_date -> Nullable<Date>,
        fp1_time -> Nullable<Time>,
        fp2_date -> Nullable<Date>,
        fp2_time -> Nullable<Time>,
        fp3_date -> Nullable<Date>,
        fp3_time -> Nullable<Time>,
        quali_date -> Nullable<Date>,
        quali_time -> Nullable<Time>,
        sprint_date -> Nullable<Date>,
        sprint_time -> Nullable<Time>,
    }
}

diesel::table! {
    results (resultId) {
        resultId -> Integer,
        raceId -> Integer,
        driverId -> Integer,
        constructorId -> Integer,
        number -> Nullable<Integer>,
        grid -> Integer,
        position -> Nullable<Integer>,
        positionText -> Varchar,
        positionOrder -> Integer,
        points -> Float,
        laps -> Integer,
        time -> Nullable<Varchar>,
        milliseconds -> Nullable<Integer>,
        fastestLap -> Nullable<Integer>,
        rank -> Nullable<Integer>,
        fastestLapTime -> Nullable<Varchar>,
        fastestLapSpeed -> Nullable<Varchar>,
        statusId -> Integer,
    }
}

diesel::table! {
    seasons (year) {
        year -> Integer,
        url -> Varchar,
    }
}

diesel::table! {
    sprintResults (sprintResultId) {
        sprintResultId -> Integer,
        raceId -> Integer,
        driverId -> Integer,
        constructorId -> Integer,
        number -> Integer,
        grid -> Integer,
        position -> Nullable<Integer>,
        positionText -> Varchar,
        positionOrder -> Integer,
        points -> Float,
        laps -> Integer,
        time -> Nullable<Varchar>,
        milliseconds -> Nullable<Integer>,
        fastestLap -> Nullable<Integer>,
        fastestLapTime -> Nullable<Varchar>,
        statusId -> Integer,
    }
}

diesel::table! {
    status (statusId) {
        statusId -> Integer,
        #[sql_name = "status"]
        statusName -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    circuits,
    constructorResults,
    constructorStandings,
    constructors,
    driverStandings,
    drivers,
    lapTimes,
    pitStops,
    qualifying,
    races,
    results,
    seasons,
    sprintResults,
    status,
);
