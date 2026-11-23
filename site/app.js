(() => {
  'use strict';

  const MONTHS = ['Baisakh','Jestha','Ashadh','Shrawan','Bhadra','Ashwin','Kartik','Mangsir','Poush','Magh','Falgun','Chaitra'];
  const MONTHS_NE = ['बैशाख','जेठ','असार','साउन','भदौ','असोज','कात्तिक','मंसिर','पुस','माघ','फागुन','चैत'];
  const WEEKDAYS = ['Sun','Mon','Tue','Wed','Thu','Fri','Sat'];
  const WEEKDAYS_NE = ['आइत','सोम','मंगल','बुध','बिही','शुक्र','शनि'];
  const DAY_NAMES = ['Aaitabar','Sombar','Mangalbar','Budhabar','Bihibar','Shukrabar','Shanibar'];
  const DAY_NAMES_NE = ['आइतबार','सोमबार','मंगलबार','बुधबार','बिहीबार','शुक्रबार','शनिबार'];
  const TEXT = {
    en: {
      skip:'Skip to calendar',navCalendar:'Calendar',navAbout:'About',navGithub:'GitHub',chooseTheme:'Choose theme',chooseThemeTitle:'Choose theme (c)',
      heroEyebrow:'NEPALI PATRO / TUI + WEB',heroTitle:'Bikram Sambat,',heroTitleAccent:'at your fingertips.',
      heroCopy:'npltz is a Nepali Patro and terminal user interface (TUI). Browse Bikram Sambat dates with their Gregorian matches in this offline browser edition.',
      windowCalendar:'Calendar',nepalTime:'Nepal time',todayInNepal:'TODAY IN NEPAL',selectedDate:'SELECTED DATE',selectedDateDetails:'Selected date details',
      gregorian:'Gregorian',dayOfYear:'Day of year',week:'Week',yearProgress:'Year progress',offlineNote:'Calendar data stays on this device',
      calendarSystem:'BIKRAM SAMBAT / NEPALI PATRO',today:'Today',specialDay:'Special day',yearOverview:'Year overview',browseMonths:'Browse months',browseYears:'Browse years',goToDate:'Go to date',keyboardHelp:'Keyboard help',
      keyboardFirst:'KEYBOARD FIRST',keyboardHeadline:'Made for the keys. Works with a click.',keyboardCopy:'Move around the calendar with arrow keys, or use the controls. Press',keyboardCopyEnd:'any time for the full guide.',seeShortcuts:'See keyboard shortcuts',builtIn:'BUILT IN',builtInCopy:'Every date is calculated from the same verified year data as the terminal app. No network call needed.',
      aboutEyebrow:'THE TUI, NOW ON THE WEB',aboutTitle:'A Nepali Patro',aboutTitleAccent:'for terminal and browser.',aboutCopy:'npltz is a free, offline Bikram Sambat calendar written in Rust with Ratatui. The TUI and this browser edition share the same calendar data, date rules and themes.',exploreTui:'Explore the TUI',getAndroid:'Get the Android app',footerTagline:'Nepali dates, quietly.',source:'Source',changelog:'Changelog',androidApp:'Android app',license:'MIT License',footerCredit:'Open source - Made in Nepal',
      makeItYours:'MAKE IT YOURS',jumpToDate:'JUMP TO A DATE',goToMonth:'Go to month',bsDate:'Bikram Sambat date',gotoPlaceholder:'2083-06 or 2083-06-15',gotoHint:'Enter year-month, or year-month-day. Years 1975 to 2090.',navigateYourWay:'NAVIGATE YOUR WAY',keyboardShortcuts:'Keyboard shortcuts',previousNextMonth:'Previous / next month',previousNextYear:'Previous / next year',moveSelectedDay:'Move selected day',thenArrows:'then arrows',goToToday:'Go to today',goToADate:'Go to a date',changeTheme:'Change theme',toggleAdDates:'Toggle Gregorian dates',thisHelp:'This help',closeDialog:'Close dialog',wholeYear:'THE WHOLE YEAR',yearHint:'change year - select a month to open it',
      close:'Close',interactiveCalendar:'Interactive Nepali calendar',calendarGrid:'Month calendar',language:'Language',footerNavigation:'Footer navigation',previousMonth:'Previous month',nextMonth:'Next month',previousMonthTitle:'Previous month (left arrow)',nextMonthTitle:'Next month (right arrow)',todayTitle:'Go to today (t)',yearOverviewTitle:'Year overview (y)',previousYear:'Previous year',nextYear:'Next year',
      rangeLimit:'Calendar data covers BS 1975-2090.',firstYear:'Reached the first calendar year.',lastYear:'Reached the last calendar year.',
      invalidDateFormat:'Use YYYY-MM or YYYY-MM-DD.',outsideCalendar:'That year or month is outside the calendar data.',monthHasDays:'{month} {year} has {days} days.',
      nepaliNewYear:'Nepali New Year',nepaliNewYearGreeting:'Wishing you a bright new year.',hariBirthday:"Developer's birthday",hariGreeting:'Happy birthday, Hari! Have a wonderful year.',gregorianNewYear:'New Year {year}',gregorianGreeting:'A brand new year begins. Happy New Year!',
      dataLoadError:'Could not load calendar data.',calendarLoadError:'Calendar data could not be loaded. Open the site from a local web server and make sure calendar_data.json is available at the repository root.',
      theme_catppuccin_mocha:'Catppuccin Mocha',theme_dracula:'Dracula',theme_everforest:'Everforest',theme_gruvbox:'Gruvbox',theme_light:'Light',theme_nord:'Nord',theme_plain_dark:'Plain dark',theme_rose_pine:'Rosé Pine',theme_warm_reading:'Warm reading',language:'Language',loading:'Loading',calendarSystemShort:'calendar',loadingCalendar:'Loading calendar data…'
    },
    ne: {
      skip:'पात्रोमा जानुहोस्',navCalendar:'पात्रो',navAbout:'परिचय',navGithub:'GitHub',chooseTheme:'थिम छान्नुहोस्',chooseThemeTitle:'थिम छान्नुहोस् (c)',
      heroEyebrow:'नेपाली पात्रो / TUI + वेब',heroTitle:'विक्रम संवत्,',heroTitleAccent:'अब तपाईंको हातमा।',
      heroCopy:'npltz नेपाली पात्रो र टर्मिनल प्रयोगकर्ता इन्टरफेस (TUI) हो। यस अफलाइन वेब संस्करणमा विक्रम संवत्का मितिसँगै इस्वी संवत्का मिति हेर्नुहोस्।',
      windowCalendar:'पात्रो',nepalTime:'नेपाल समय',todayInNepal:'आज नेपालमा',selectedDate:'छानिएको मिति',selectedDateDetails:'छानिएको मितिको विवरण',
      gregorian:'इस्वी संवत्',dayOfYear:'वर्षको दिन',week:'हप्ता',yearProgress:'वर्षको प्रगति',offlineNote:'पात्रोको डेटा यही उपकरणमा रहन्छ',
      calendarSystem:'विक्रम संवत् / नेपाली पात्रो',today:'आज',specialDay:'विशेष दिन',yearOverview:'वर्ष दृश्य',browseMonths:'महिना हेर्नुहोस्',browseYears:'वर्ष हेर्नुहोस्',goToDate:'मितिमा जानुहोस्',keyboardHelp:'किबोर्ड सहयोग',
      keyboardFirst:'किबोर्डबाट चलाउनुहोस्',keyboardHeadline:'किबोर्डबाट सजिलै। क्लिक गरेर पनि।',keyboardCopy:'एरो की वा बटनबाट पात्रो चलाउनुहोस्। पूरा सहयोगका लागि',keyboardCopyEnd:'थिच्नुहोस्।',seeShortcuts:'किबोर्ड सर्टकट हेर्नुहोस्',builtIn:'पहिल्यै समावेश',builtInCopy:'टर्मिनल एपमा जस्तै प्रमाणित पात्रो डेटा प्रयोग हुन्छ। इन्टरनेट चाहिँदैन।',
      aboutEyebrow:'TUI अब वेबमा',aboutTitle:'नेपाली पात्रो',aboutTitleAccent:'टर्मिनल र ब्राउजरमा।',aboutCopy:'npltz Rust र Ratatui मा बनेको निःशुल्क, अफलाइन विक्रम संवत् पात्रो हो। TUI र यो वेब संस्करणमा एउटै पात्रो डेटा, मिति नियम र थिम प्रयोग हुन्छन्।',exploreTui:'TUI हेर्नुहोस्',getAndroid:'Android एप लिनुहोस्',footerTagline:'नेपाली मिति, सरल रूपमा।',source:'स्रोत कोड',changelog:'परिवर्तन विवरण',androidApp:'Android एप',license:'MIT इजाजतपत्र',footerCredit:'खुला स्रोत - नेपालमा निर्मित',
      makeItYours:'आफ्नो थिम छान्नुहोस्',jumpToDate:'मितिमा जानुहोस्',goToMonth:'महिना खोल्नुहोस्',bsDate:'विक्रम संवत् मिति',gotoPlaceholder:'२०८३-०६ वा २०८३-०६-१५',gotoHint:'वर्ष-महिना वा वर्ष-महिना-दिन लेख्नुहोस्। वर्ष १९७५ देखि २०९० सम्म।',navigateYourWay:'किबोर्डबाट चलाउनुहोस्',keyboardShortcuts:'किबोर्ड सर्टकट',previousNextMonth:'अघिल्लो / अर्को महिना',previousNextYear:'अघिल्लो / अर्को वर्ष',moveSelectedDay:'छानिएको दिन सार्नुहोस्',thenArrows:'पछि एरो की',goToToday:'आजमा जानुहोस्',goToADate:'मितिमा जानुहोस्',changeTheme:'थिम बदल्नुहोस्',toggleAdDates:'इस्वी मिति देखाउनुहोस् / लुकाउनुहोस्',thisHelp:'यो सहयोग',closeDialog:'झ्याल बन्द गर्नुहोस्',wholeYear:'पूरा वर्ष',yearHint:'वर्ष बदल्नुहोस् - महिना छानेर खोल्नुहोस्',
      close:'बन्द गर्नुहोस्',interactiveCalendar:'अन्तर्क्रियात्मक नेपाली पात्रो',calendarGrid:'महिनाको पात्रो',language:'भाषा',footerNavigation:'फुटर नेभिगेसन',previousMonth:'अघिल्लो महिना',nextMonth:'अर्को महिना',previousMonthTitle:'अघिल्लो महिना (बायाँ एरो)',nextMonthTitle:'अर्को महिना (दायाँ एरो)',todayTitle:'आजमा जानुहोस् (t)',yearOverviewTitle:'वर्ष दृश्य (y)',previousYear:'अघिल्लो वर्ष',nextYear:'अर्को वर्ष',
      rangeLimit:'पात्रो डेटा वि.सं. १९७५-२०९० सम्म छ।',firstYear:'पात्रोको पहिलो वर्षमा पुग्नुभयो।',lastYear:'पात्रोको अन्तिम वर्षमा पुग्नुभयो।',
      invalidDateFormat:'वर्ष-महिना वा वर्ष-महिना-दिन ढाँचामा लेख्नुहोस्।',outsideCalendar:'यो वर्ष वा महिना पात्रो डेटामा छैन।',monthHasDays:'{year} सालको {month} महिनामा {days} दिन छन्।',
      nepaliNewYear:'नेपाली नयाँ वर्ष',nepaliNewYearGreeting:'नयाँ वर्षको हार्दिक शुभकामना।',hariBirthday:'विकासकर्ताको जन्मदिन',hariGreeting:'हरि, जन्मदिनको हार्दिक शुभकामना!',gregorianNewYear:'नयाँ वर्ष {year}',gregorianGreeting:'नयाँ वर्षको हार्दिक शुभकामना!',
      dataLoadError:'पात्रो डेटा लोड भएन।',calendarLoadError:'पात्रो डेटा लोड भएन। स्थानीय वेब सर्भरबाट साइट खोल्नुहोस् र calendar_data.json रिपोजिटरीको मूल फोल्डरमा छ भनी जाँच्नुहोस्।',
      theme_catppuccin_mocha:'Catppuccin Mocha',theme_dracula:'Dracula',theme_everforest:'Everforest',theme_gruvbox:'Gruvbox',theme_light:'उज्यालो',theme_nord:'Nord',theme_plain_dark:'सादा गाढा',theme_rose_pine:'Rosé Pine',theme_warm_reading:'न्यानो पठन',language:'भाषा',loading:'लोड हुँदैछ',calendarSystemShort:'पात्रो',loadingCalendar:'पात्रो डेटा लोड हुँदैछ…'
    }
  };

  const THEMES = [
    ['catppuccin-mocha','Catppuccin Mocha',['#1e1e2e','#89b4fa','#add8be']],['dracula','Dracula',['#282a36','#bd93f9','#8be9fd']],
    ['everforest','Everforest',['#2d353b','#a7c080','#7fbbb3']],['gruvbox','Gruvbox',['#282828','#b8bb26','#83a598']],
    ['light','Light',['#f6f6f4','#3f5b9e','#5e6e7e']],['nord','Nord',['#2e3440','#88c0d0','#a3be8c']],
    ['plain-dark','Plain dark',['#000000','#aaaaaa','#888888']],['rose-pine','Rosé Pine',['#191724','#c4a7e7','#9ccfd8']],
    ['warm-reading','Warm reading',['#f3e9d6','#9c5a32','#83705a']],
  ];
  const EPOCH = Date.UTC(1918, 3, 13);
  const $ = (id) => document.getElementById(id);
  const state = { lang: 'en', data: null, years: [], yearIndex: new Map(), viewYear: 0, viewMonth: 0, yearView: 0, selectedDay: 1, today: null, showAd: true, cursor: false, toastTimer: 0 };

  const utcDate = (year, month, day) => new Date(Date.UTC(year, month - 1, day));
  const dayOffset = (date) => Math.round((Date.UTC(date.getUTCFullYear(), date.getUTCMonth(), date.getUTCDate()) - EPOCH) / 86400000);
  const adAtOffset = (offset) => new Date(EPOCH + offset * 86400000);
  const pad = (n) => String(n).padStart(2, '0');
  const dateKey = (y, m, d) => `${y}-${pad(m)}-${pad(d)}`;
  const yearData = (year) => state.yearIndex.get(year);
  function supportsFlexGap() {
    const flex = document.createElement('div');
    flex.style.cssText = 'display:flex;flex-direction:column;position:absolute;top:-1000px;left:-1000px;row-gap:1px';
    const first = document.createElement('div');
    const second = document.createElement('div');
    first.style.height = '1px';
    second.style.height = '1px';
    flex.appendChild(first);
    flex.appendChild(second);
    document.body.appendChild(flex);
    const supported = flex.scrollHeight === 3;
    document.body.removeChild(flex);
    return supported;
  }
  const daysInMonth = (year, month) => { const data = yearData(year); return data ? data.months[month - 1] : 0; };
  const yearTotal = (year) => { const data = yearData(year); return data ? data.total_days : 0; };
  const tr = (key) => { const languageText = TEXT[state.lang]; return (languageText && languageText[key]) || TEXT.en[key] || key; };
  const locale = () => state.lang === 'ne' ? 'ne-NP' : 'en-US';
  const toNepaliDigits = (value) => String(value).replace(/[0-9]/g, (digit) => '०१२३४५६७८९'[Number(digit)]);
  const formatNumber = (n) => state.lang === 'ne' ? toNepaliDigits(n) : new Intl.NumberFormat('en-US', {useGrouping:false}).format(n);
  const formatAdDate = (date, options = {}) => {
    const fmt = new Intl.DateTimeFormat(locale(), {timeZone:'UTC', month:options.month || 'short', day:'numeric'});
    const parts = Object.fromEntries(fmt.formatToParts(date).map(({type,value}) => [type,value]));
    return `${options.weekday ? `${new Intl.DateTimeFormat(locale(),{timeZone:'UTC',weekday:'long'}).format(date)} ` : ''}${parts.month} ${parts.day} ${formatNumber(date.getUTCFullYear())}`;
  };
  const normalizeDigits = (value) => value.replace(/[०-९]/g, (digit) => String('०१२३४५६७८९'.indexOf(digit)));
  const monthName = (month) => (state.lang === 'ne' ? MONTHS_NE : MONTHS)[month - 1];
  const weekdayName = (index) => (state.lang === 'ne' ? DAY_NAMES_NE : DAY_NAMES)[index];
  const interpolate = (value, vars = {}) => value.replace(/\{(\w+)\}/g, (_, key) => vars[key] == null ? '' : vars[key]);

  function applyLanguage(lang) {
    state.lang = lang === 'ne' ? 'ne' : 'en';
    document.documentElement.lang = state.lang;
    try { localStorage.setItem('npltz-web-lang', state.lang); } catch {}
    document.querySelectorAll('[data-i18n]').forEach((el) => { el.textContent = tr(el.dataset.i18n); });
    document.querySelectorAll('[data-i18n-aria]').forEach((el) => { el.setAttribute('aria-label', tr(el.dataset.i18nAria)); });
    document.querySelectorAll('[data-i18n-title]').forEach((el) => { el.title = tr(el.dataset.i18nTitle); });
    document.querySelectorAll('[data-i18n-placeholder]').forEach((el) => { el.placeholder = tr(el.dataset.i18nPlaceholder); });
    document.querySelectorAll('[data-language]').forEach((button) => button.setAttribute('aria-pressed', String(button.dataset.language === state.lang)));
    document.querySelectorAll('.theme-option').forEach((button) => { const label=button.querySelector('span:nth-child(2)'); if(label)label.textContent=tr(`theme_${button.dataset.theme.replace(/-/g, '_')}`); });
    document.title = state.lang === 'ne' ? 'npltz - नेपाली पात्रो' : 'npltz - Nepali calendar';
    document.querySelector('meta[name="description"]').content = tr('heroCopy');
    const todayLabel = $('side-label');
    const todayBs = state.today ? getTodayBs() : null;
    if (todayLabel) todayLabel.textContent = todayBs && state.viewYear === todayBs.year && state.viewMonth === todayBs.month && state.selectedDay === todayBs.day ? tr('todayInNepal') : tr('selectedDate');
    updateNepalClock();
    if (state.years.length) render();
    const yearDialog = $('year-dialog');
    if (yearDialog && yearDialog.open) renderYear();
  }

  function bsToAd(year, month, day) {
    const yd = yearData(year);
    if (!yd || month < 1 || month > 12 || day < 1 || day > yd.months[month - 1]) return null;
    let offset = 0;
    for (let y = state.years[0].year; y < year; y++) offset += yearTotal(y);
    for (let m = 1; m < month; m++) offset += daysInMonth(year, m);
    return adAtOffset(offset + day - 1);
  }

  function adToBs(date) {
    let remaining = dayOffset(date);
    if (remaining < 0) return null;
    for (const yd of state.years) {
      if (remaining < yd.total_days) {
        let month = 1;
        for (const count of yd.months) {
          if (remaining < count) return { year: yd.year, month, day: remaining + 1 };
          remaining -= count;
          month++;
        }
        return null;
      }
      remaining -= yd.total_days;
    }
    return null;
  }

  function nepalToday() {
    const parts = new Intl.DateTimeFormat('en-CA', { timeZone: 'Asia/Kathmandu', year:'numeric', month:'2-digit', day:'2-digit' }).formatToParts(new Date());
    const part = Object.fromEntries(parts.map(({type,value}) => [type,value]));
    return utcDate(Number(part.year), Number(part.month), Number(part.day));
  }

  function getTodayBs() { return adToBs(state.today); }
  function monthStartOffset(year, month) {
    const first = bsToAd(year, month, 1);
    return first ? first.getUTCDay() : 0;
  }
  function dayOfYear(year, month, day) {
    let n = day;
    for (let m = 1; m < month; m++) n += daysInMonth(year, m);
    return n;
  }
  function specialDay(year, month, day) {
    if (month === 1 && day === 1) return [tr('nepaliNewYear'),tr('nepaliNewYearGreeting')];
    if (month === 5 && day === 30) return [tr('hariBirthday'),tr('hariGreeting')];
    const ad = bsToAd(year, month, day);
    if (ad && ad.getUTCMonth() === 0 && ad.getUTCDate() === 1) return [interpolate(tr('gregorianNewYear'),{year:formatNumber(ad.getUTCFullYear())}),tr('gregorianGreeting')];
    return null;
  }

  function setSelected(year, month, day, changeView = true) {
    const max = daysInMonth(year, month);
    if (!max) return false;
    day = Math.max(1, Math.min(day, max));
    if (changeView) { state.viewYear = year; state.viewMonth = month; }
    state.selectedDay = day;
    render();
    return true;
  }

  function navigateMonth(delta) {
    let y = state.viewYear, m = state.viewMonth + delta;
    if (m < 1) { y--; m = 12; } else if (m > 12) { y++; m = 1; }
    if (!yearData(y)) { showToast(tr('rangeLimit')); return; }
    const day = Math.min(state.selectedDay, daysInMonth(y,m));
    setSelected(y,m,day);
  }
  function navigateYear(delta) {
    const y = state.viewYear + delta;
    if (!yearData(y)) { showToast(tr('rangeLimit')); return; }
    setSelected(y,state.viewMonth,Math.min(state.selectedDay,daysInMonth(y,state.viewMonth)));
  }
  function goToday() {
    const td = getTodayBs();
    if (td) setSelected(td.year,td.month,td.day);
  }

  function renderMonth() {
    const {viewYear: year, viewMonth: month} = state;
    $('month-title').textContent = monthName(month);
    $('year-title').textContent = formatNumber(year);
    const start = bsToAd(year,month,1), end = bsToAd(year,month,daysInMonth(year,month));
    $('ad-range').textContent = start && end ? `${formatAdDate(start)} - ${formatAdDate(end)}` : '';
    const grid = $('calendar-grid');
    grid.classList.toggle('hide-ad',!state.showAd);
    while(grid.firstChild)grid.removeChild(grid.firstChild);
    for (let i=0;i<7;i++) {
      const label=document.createElement('div'); label.className=`weekday${i===0?' sun':i===6?' sat':''}`; label.setAttribute('role','columnheader'); label.textContent=state.lang==='ne'?WEEKDAYS_NE[i]:WEEKDAYS[i]; grid.append(label);
    }
    const offset=monthStartOffset(year,month), count=daysInMonth(year,month);
    for(let i=0;i<42;i++){
      const day=i-offset+1, cell=document.createElement('button');
      if(day<1||day>count){cell.type='button';cell.className='day-cell outside';cell.disabled=true;cell.setAttribute('aria-hidden','true');grid.append(cell);continue;}
      const special=specialDay(year,month,day), ad=bsToAd(year,month,day), isToday=!!(state.today&&ad&&dayOffset(ad)===dayOffset(state.today));
      cell.type='button';cell.className='day-cell';cell.setAttribute('role','gridcell');cell.setAttribute('aria-selected',String(day===state.selectedDay));
      if(day===state.selectedDay)cell.classList.add('selected'); if(isToday)cell.classList.add('today'); if(i%7===0)cell.classList.add('sunday'); if(i%7===6)cell.classList.add('saturday'); if(special)cell.classList.add('special');
      const adLabel=ad?formatNumber(ad.getUTCDate()):'';
      cell.setAttribute('aria-label',`${monthName(month)} ${formatNumber(day)} ${formatNumber(year)}${ad?`, ${formatAdDate(ad,{month:'long',weekday:true})}`:''}${isToday?`, ${tr('today')}`:''}${special?`, ${special[0]}`:''}`);
      cell.innerHTML=`<span class="bs-day">${formatNumber(day)}</span><span class="ad-day">${adLabel||'-'}</span>`;
      cell.addEventListener('click',()=>{state.cursor=false;setSelected(year,month,day);}); grid.append(cell);
    }
    const todayBs=getTodayBs();
    if(todayBs && yearData(todayBs.year)){
      const t=bsToAd(todayBs.year,todayBs.month,1), e=bsToAd(todayBs.year,todayBs.month,daysInMonth(todayBs.year,todayBs.month));
      $('data-range').textContent=`${formatNumber(state.years[0].year)}-${formatNumber(state.years[state.years.length - 1].year)}`;
    }
  }

  function renderDetails() {
    const y=state.viewYear,m=state.viewMonth,d=state.selectedDay,ad=bsToAd(y,m,d),td=getTodayBs();
    $('side-label').textContent=td&&td.year===y&&td.month===m&&td.day===d?tr('todayInNepal'):tr('selectedDate');
    $('side-day').textContent=formatNumber(d); $('side-month').textContent=monthName(m); $('side-year').textContent=formatNumber(y);
    $('side-weekday').textContent=ad?weekdayName(ad.getUTCDay()):'-';
    $('side-ad').textContent=ad?formatAdDate(ad):'-';
    const ordinal=dayOfYear(y,m,d), total=yearTotal(y), weeks=Math.ceil(total/7), pct=Math.floor(ordinal/total*100);
    $('side-dayofyear').textContent=`${formatNumber(ordinal)} / ${formatNumber(total)}`; $('side-week').textContent=`${formatNumber(Math.ceil(ordinal/7))} / ${formatNumber(weeks)}`;
    $('side-progress-label').textContent=`${formatNumber(pct)}%`; $('side-progress').style.width=`${Math.min(100,ordinal/total*100)}%`;
    const special=specialDay(y,m,d), card=$('special-card'); card.hidden=!special;
    if(special){$('special-title').textContent=special[0];$('special-copy').textContent=special[1];}
  }
  function renderYear() {
    $('year-heading').textContent=state.lang==='ne'?`${formatNumber(state.yearView)} वि.सं.`:`${formatNumber(state.yearView)} BS`;
    const root=$('year-grid');while(root.firstChild)root.removeChild(root.firstChild);
    for(let month=1;month<=12;month++){
      const box=document.createElement('section');box.className='mini-month';
      const title=document.createElement('button');title.className='mini-title';title.textContent=monthName(month);title.addEventListener('click',()=>{setSelected(state.yearView,month,1);$('year-dialog').close();});box.append(title);
      const days=document.createElement('div');days.className='mini-days';
      WEEKDAYS.forEach((w,i)=>{const h=document.createElement('span');h.className=`mini-head${i===0?' sun':i===6?' sat':''}`;h.textContent=state.lang==='ne'?['आ','सो','मं','बु','बि','शु','श'][i]:w[0];days.append(h);});
      const offset=monthStartOffset(state.yearView,month),count=daysInMonth(state.yearView,month);
      for(let i=0;i<42;i++){
        const d=i-offset+1;
        if(d<1||d>count){const blank=document.createElement('span');blank.className='mini-empty';days.append(blank);continue;}
        const b=document.createElement('button');b.className=`mini-day${i%7===0?' sun':i%7===6?' sat':''}`;b.textContent=formatNumber(d);
        const ad=bsToAd(state.yearView,month,d);if(state.today&&ad&&dayOffset(ad)===dayOffset(state.today))b.classList.add('today');if(specialDay(state.yearView,month,d))b.classList.add('special');
        b.setAttribute('aria-label',`${monthName(month)} ${formatNumber(d)}`);b.addEventListener('click',()=>{setSelected(state.yearView,month,d);$('year-dialog').close();});days.append(b);
      }
      box.append(days);root.append(box);
    }
  }
  function render() {renderMonth();renderDetails();}

  function showToast(message){const t=$('toast');t.textContent=message;t.classList.add('show');clearTimeout(state.toastTimer);state.toastTimer=setTimeout(()=>t.classList.remove('show'),2300);}
  function openDialog(id){const d=$(id);if(!d.open)d.showModal();}
  function closeDialog(dialog){if(dialog.open)dialog.close();}
  function setTheme(name){if(!THEMES.some(([id])=>id===name))return;document.documentElement.dataset.theme=name;try{localStorage.setItem('npltz-web-theme',name);}catch{}document.querySelectorAll('.theme-option').forEach(b=>b.setAttribute('aria-pressed',String(b.dataset.theme===name)));const meta=document.querySelector('meta[name="theme-color"]');meta.content=getComputedStyle(document.documentElement).getPropertyValue('--bg').trim();}
  function setupThemeDialog(){const list=$('theme-list');THEMES.forEach(([id,name,colors])=>{const b=document.createElement('button');b.type='button';b.className='theme-option';b.dataset.theme=id;b.setAttribute('aria-pressed',String(document.documentElement.dataset.theme===id));const dots=colors.map(c=>`<i style="background:${c}"></i>`).join('');b.innerHTML=`<span class="swatches" aria-hidden="true">${dots}</span><span>${tr(`theme_${id.replace(/-/g, '_')}`)}</span><span class="theme-check" aria-hidden="true">✓</span>`;b.addEventListener('click',()=>{setTheme(id);closeDialog($('theme-dialog'));});list.append(b);});}
  function setupDialogs(){
    [['theme-open','theme-dialog'],['help-open','help-dialog']].forEach(([button,dialog])=>$(button).addEventListener('click',()=>openDialog(dialog)));
    $('year-toggle').addEventListener('click',()=>{state.yearView=state.viewYear;renderYear();openDialog('year-dialog');});
    document.querySelectorAll('dialog .dialog-close').forEach(b=>b.addEventListener('click',()=>closeDialog(b.closest('dialog'))));
    document.querySelectorAll('dialog').forEach(d=>d.addEventListener('click',e=>{if(e.target===d)d.close();}));
    $('year-prev').addEventListener('click',()=>{if(yearData(state.yearView-1)){state.yearView--;renderYear();}else showToast(tr('firstYear'));});
    $('year-next').addEventListener('click',()=>{if(yearData(state.yearView+1)){state.yearView++;renderYear();}else showToast(tr('lastYear'));});
    $('goto-form').addEventListener('submit',e=>{e.preventDefault();const raw=normalizeDigits($('goto-input').value.trim());const match=raw.match(/^(\d{4})-(\d{1,2})(?:-(\d{1,2}))?$/);const err=$('goto-error');
      if(!match){err.textContent=tr('invalidDateFormat');return;}const y=Number(match[1]),m=Number(match[2]),d=Number(match[3]||1),max=daysInMonth(y,m);
      if(!max){err.textContent=tr('outsideCalendar');return;}if(d<1||d>max){err.textContent=interpolate(tr('monthHasDays'),{month:monthName(m),year:formatNumber(y),days:formatNumber(max)});return;}setSelected(y,m,d);err.textContent='';closeDialog($('goto-dialog'));});
  }
  function setupControls(){
    document.querySelectorAll('[data-language]').forEach((button)=>button.addEventListener('click',()=>applyLanguage(button.dataset.language)));
    $('prev-month').addEventListener('click',()=>navigateMonth(-1));$('next-month').addEventListener('click',()=>navigateMonth(1));$('today-button').addEventListener('click',goToday);
    document.addEventListener('keydown',e=>{
      if(e.altKey||e.ctrlKey||e.metaKey)return;
      const open=document.querySelector('dialog[open]');
      if(open){if(e.key==='Escape'){closeDialog(open);e.preventDefault();return;}if(open.id==='year-dialog'&&e.key==='ArrowLeft'){e.preventDefault();$('year-prev').click();}if(open.id==='year-dialog'&&e.key==='ArrowRight'){e.preventDefault();$('year-next').click();}return;}
      const tag=e.target && e.target.tagName;if(tag==='INPUT'||tag==='TEXTAREA'||(e.target && e.target.isContentEditable))return;
      const key=e.key;
      if(state.cursor&&(key==='ArrowLeft'||key==='h'||key==='ArrowRight'||key==='l'||key==='ArrowUp'||key==='k'||key==='ArrowDown'||key==='j')){
        let step=(key==='ArrowLeft'||key==='h')?-1:(key==='ArrowRight'||key==='l')?1:(key==='ArrowUp'||key==='k')?-7:7;
        let next=state.selectedDay+step,y=state.viewYear,m=state.viewMonth;
        if(next<1){if(--m<1){m=12;y--;}next+=daysInMonth(y,m);}else if(next>daysInMonth(y,m)){next-=daysInMonth(y,m);if(++m>12){m=1;y++;}}
        if(yearData(y))setSelected(y,m,next);e.preventDefault();const selected=document.querySelector('.day-cell.selected');if(selected)selected.focus();
      }
      else if(key==='ArrowLeft'||key==='h'){navigateMonth(-1);e.preventDefault();}
      else if(key==='ArrowRight'||key==='l'){navigateMonth(1);e.preventDefault();}
      else if(key==='ArrowUp'||key==='k'){navigateYear(1);e.preventDefault();}
      else if(key==='ArrowDown'||key==='j'){navigateYear(-1);e.preventDefault();}
      else if(key==='t'){goToday();e.preventDefault();}
      else if(key==='c'){openDialog('theme-dialog');e.preventDefault();}
      else if(key==='g'){const input=$('goto-input');input.value='';$('goto-error').textContent='';openDialog('goto-dialog');setTimeout(()=>input.focus(),0);e.preventDefault();}
      else if(key==='?'){openDialog('help-dialog');e.preventDefault();}
      else if(key==='y'){state.yearView=state.viewYear;renderYear();openDialog('year-dialog');e.preventDefault();}
      else if(key==='a'){state.showAd=!state.showAd;renderMonth();e.preventDefault();}
      else if(key==='Tab'&&!e.shiftKey){state.cursor=true;const selected=document.querySelector('.day-cell.selected');if(selected)selected.focus();e.preventDefault();}
      else if(state.cursor&&key==='Tab'){state.cursor=false;e.preventDefault();}
      else if(key==='Enter'&&state.cursor){const s=specialDay(state.viewYear,state.viewMonth,state.selectedDay);if(s)showToast(`${s[0]} · ${s[1]}`);}
    });
  }

  function updateNepalClock(){
    const clock=$('nepal-time');
    if(clock)clock.textContent=new Intl.DateTimeFormat(locale(),{timeZone:'Asia/Kathmandu',hour:'numeric',minute:'2-digit',second:'2-digit',hour12:true}).format(new Date());
    const today=nepalToday();
    if(state.years.length&&(!state.today||dayOffset(today)!==dayOffset(state.today))){state.today=today;render();}
  }

  async function init(){
    if (!supportsFlexGap()) document.documentElement.classList.add('no-flex-gap');
    try{state.lang=localStorage.getItem('npltz-web-lang')==='ne'?'ne':'en';}catch{}
    applyLanguage(state.lang);setupThemeDialog();setupDialogs();setupControls();updateNepalClock();setInterval(updateNepalClock,1000);
    document.addEventListener('visibilitychange',()=>{if(!document.hidden)updateNepalClock();});window.addEventListener('focus',updateNepalClock);
    try{
      let response=await fetch('../calendar_data.json',{cache:'force-cache'});
      if(!response.ok)response=await fetch('calendar_data.json',{cache:'force-cache'});
      if(!response.ok)throw new Error('Calendar data request failed');
      const data=await response.json();if(!Array.isArray(data)||!data.length)throw new Error('Calendar data is empty');
      state.years=data;state.yearIndex=new Map(data.map(y=>[y.year,y]));
      const today=nepalToday(),todayBs=adToBs(today);state.today=today;
      if(!todayBs)throw new Error('Today is outside the available calendar data');
      state.viewYear=todayBs.year;state.viewMonth=todayBs.month;state.selectedDay=todayBs.day;
      render();
    }catch(error){console.error(error);$('side-weekday').textContent=tr('dataLoadError');$('calendar-grid').innerHTML=`<p class="load-error">${tr('calendarLoadError')}</p>`;showToast(tr('dataLoadError'));}
  }
  init();
})();
