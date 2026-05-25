/* ==========================================================================
   Breeze Retention Suite — Marketing Site Scripts
   ========================================================================== */

(function () {
  'use strict';

  // ---------- Intersection Observer for fade-up animations ----------
  var observer = new IntersectionObserver(
    function (entries) {
      entries.forEach(function (entry) {
        if (entry.isIntersecting) {
          entry.target.classList.add('visible');
        }
      });
    },
    { threshold: 0.08, rootMargin: '0px 0px -40px 0px' }
  );

  document.querySelectorAll('.animate-on-scroll').forEach(function (el) {
    observer.observe(el);
  });

  // ---------- Sticky nav background on scroll ----------
  var nav = document.querySelector('.nav');
  var lastScrollY = 0;

  function onScroll() {
    var scrollY = window.scrollY;
    if (nav) {
      nav.classList.toggle('scrolled', scrollY > 10);
    }
    lastScrollY = scrollY;
  }

  window.addEventListener('scroll', onScroll, { passive: true });
  onScroll();

  // ---------- Mobile nav toggle ----------
  var toggle = document.querySelector('.nav__toggle');
  var links = document.querySelector('.nav__links');

  if (toggle && links) {
    toggle.addEventListener('click', function () {
      links.classList.toggle('open');
      toggle.setAttribute(
        'aria-expanded',
        links.classList.contains('open').toString()
      );
    });

    // Close mobile nav when a link is clicked
    links.querySelectorAll('a').forEach(function (link) {
      link.addEventListener('click', function () {
        links.classList.remove('open');
        toggle.setAttribute('aria-expanded', 'false');
      });
    });
  }

  // ---------- Smooth scroll for anchor links ----------
  document.querySelectorAll('a[href^="#"]').forEach(function (anchor) {
    anchor.addEventListener('click', function (e) {
      var target = document.querySelector(this.getAttribute('href'));
      if (target) {
        e.preventDefault();
        var offset = parseInt(
          getComputedStyle(document.documentElement)
            .getPropertyValue('--nav-height'),
          10
        ) || 48;
        var top =
          target.getBoundingClientRect().top + window.pageYOffset - offset;
        window.scrollTo({ top: top, behavior: 'smooth' });
      }
    });
  });

  // ---------- Spin-the-wheel animation ----------
  var spinWheel = document.querySelector('.spin-mockup');
  var spinObserver = new IntersectionObserver(
    function (entries) {
      entries.forEach(function (entry) {
        if (entry.isIntersecting) {
          entry.target.style.animation = 'spinOnce 2s ease-out forwards';
          spinObserver.unobserve(entry.target);
        }
      });
    },
    { threshold: 0.5 }
  );

  if (spinWheel) {
    spinObserver.observe(spinWheel);
  }

  // Inject spin keyframes
  var style = document.createElement('style');
  style.textContent =
    '@keyframes spinOnce { from { transform: rotate(0deg); } to { transform: rotate(720deg); } }';
  document.head.appendChild(style);

  // ---------- Counter animation for stat numbers ----------
  var statNumbers = document.querySelectorAll('[data-count]');
  var countObserver = new IntersectionObserver(
    function (entries) {
      entries.forEach(function (entry) {
        if (entry.isIntersecting) {
          animateCount(entry.target);
          countObserver.unobserve(entry.target);
        }
      });
    },
    { threshold: 0.5 }
  );

  statNumbers.forEach(function (el) {
    countObserver.observe(el);
  });

  function animateCount(el) {
    var text = el.getAttribute('data-count');
    var prefix = el.getAttribute('data-prefix') || '';
    var suffix = el.getAttribute('data-suffix') || '';

    var numericPart = text.replace(/[^0-9.]/g, '');
    var targetNum = parseFloat(numericPart);

    if (isNaN(targetNum)) {
      el.textContent = prefix + text + suffix;
      return;
    }

    var duration = 1200;
    var startTime = null;
    var isFloat = numericPart.includes('.');

    function step(timestamp) {
      if (!startTime) startTime = timestamp;
      var progress = Math.min((timestamp - startTime) / duration, 1);
      var eased = 1 - Math.pow(1 - progress, 3);
      var current = eased * targetNum;

      if (isFloat) {
        el.textContent = prefix + current.toFixed(0) + suffix;
      } else {
        el.textContent = prefix + Math.floor(current).toLocaleString('en-IN') + suffix;
      }

      if (progress < 1) {
        requestAnimationFrame(step);
      } else {
        el.textContent = prefix + text + suffix;
      }
    }

    requestAnimationFrame(step);
  }
})();
